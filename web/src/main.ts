// signgeom web demo — renders a straight-line geodesic, integrated under
// four metric signatures. When a WebGPU adapter is available we run the
// kernel in `shader.wgsl` and overlay the result on top of the CPU path
// as a sanity-check (the two should agree to single-precision tolerance).
// When WebGPU is unavailable we silently fall back to the CPU-only path.

import shaderSource from "./shader.wgsl?raw";
import { integrateFlat, PathSample } from "./integrator";
import {
  SIGNATURES,
  Signature,
  SignatureKey,
  canonicalDiagonal,
} from "./signature";

const status = document.getElementById("status") as HTMLPreElement;
const canvas = document.getElementById("stage") as HTMLCanvasElement;
const runBtn = document.getElementById("run") as HTMLButtonElement;
const v0Input = document.getElementById("v0") as HTMLInputElement;
const taumaxInput = document.getElementById("taumax") as HTMLInputElement;
const stepsInput = document.getElementById("steps") as HTMLInputElement;

interface GpuCtx {
  device: GPUDevice;
  pipeline: GPUComputePipeline;
  bindGroupLayout: GPUBindGroupLayout;
}

let gpu: GpuCtx | null = null;

async function initWebGPU(): Promise<void> {
  const gpuApi: GPU | undefined = (navigator as Navigator & { gpu?: GPU }).gpu;
  if (!gpuApi) {
    log("WebGPU not available — CPU path only.");
    return;
  }
  const adapter = await gpuApi.requestAdapter();
  if (!adapter) {
    log("No WebGPU adapter — CPU path only.");
    return;
  }
  const device = await adapter.requestDevice();
  const module = device.createShaderModule({ code: shaderSource });
  const bindGroupLayout = device.createBindGroupLayout({
    entries: [
      { binding: 0, visibility: GPUShaderStage.COMPUTE, buffer: { type: "uniform" } },
      { binding: 1, visibility: GPUShaderStage.COMPUTE, buffer: { type: "storage" } },
    ],
  });
  const layout = device.createPipelineLayout({ bindGroupLayouts: [bindGroupLayout] });
  const pipeline = device.createComputePipeline({
    layout,
    compute: { module, entryPoint: "integrate" },
  });
  gpu = { device, pipeline, bindGroupLayout };
  log(`WebGPU adapter ready (${adapter.info?.description ?? "(unnamed)"}); GPU path active.`);
}

function log(line: string): void {
  status.textContent = `${line}\n${status.textContent ?? ""}`.slice(0, 4096);
}

function currentSignature(): SignatureKey {
  const checked = document.querySelector<HTMLInputElement>('input[name="sig"]:checked');
  return (checked?.value as SignatureKey) ?? "minkowski";
}

interface GpuRun {
  samples: PathSample[];
  millis: number;
}

async function runWebGPU(
  sig: Signature,
  v0: ReadonlyArray<number>,
  tauMax: number,
  steps: number,
): Promise<GpuRun | null> {
  if (!gpu) return null;
  const { device, pipeline, bindGroupLayout } = gpu;

  // Params: eta vec4, v0 vec4, steps u32, tau_max f32, _pad u32, u32 -> 12 floats / ints.
  const eta = canonicalDiagonal(sig);
  const paramArray = new Float32Array(12);
  paramArray.set(eta, 0);
  paramArray[4] = v0[0] ?? 0;
  paramArray[5] = v0[1] ?? 0;
  paramArray[6] = v0[2] ?? 0;
  paramArray[7] = v0[3] ?? 0;
  // steps as u32 reinterpreted into the same backing buffer:
  const paramU32 = new Uint32Array(paramArray.buffer);
  paramU32[8] = steps;
  paramArray[9] = tauMax;
  paramU32[10] = 0;
  paramU32[11] = 0;

  const paramBuf = device.createBuffer({
    size: paramArray.byteLength,
    usage: GPUBufferUsage.UNIFORM | GPUBufferUsage.COPY_DST,
  });
  device.queue.writeBuffer(paramBuf, 0, paramArray);

  // Each Sample = vec4 position + vec4 velocity + 4 f32 trailing fields = 12 f32 = 48 bytes.
  const sampleStride = 48;
  const totalBytes = sampleStride * (steps + 1);
  const storage = device.createBuffer({
    size: totalBytes,
    usage: GPUBufferUsage.STORAGE | GPUBufferUsage.COPY_SRC,
  });
  const readBack = device.createBuffer({
    size: totalBytes,
    usage: GPUBufferUsage.COPY_DST | GPUBufferUsage.MAP_READ,
  });

  const bindGroup = device.createBindGroup({
    layout: bindGroupLayout,
    entries: [
      { binding: 0, resource: { buffer: paramBuf } },
      { binding: 1, resource: { buffer: storage } },
    ],
  });

  const encoder = device.createCommandEncoder();
  const pass = encoder.beginComputePass();
  pass.setPipeline(pipeline);
  pass.setBindGroup(0, bindGroup);
  pass.dispatchWorkgroups(Math.ceil((steps + 1) / 64));
  pass.end();
  encoder.copyBufferToBuffer(storage, 0, readBack, 0, totalBytes);

  const t0 = performance.now();
  device.queue.submit([encoder.finish()]);
  await readBack.mapAsync(GPUMapMode.READ);
  const t1 = performance.now();
  const copy = readBack.getMappedRange().slice(0);
  readBack.unmap();
  paramBuf.destroy();
  storage.destroy();

  const view = new Float32Array(copy);
  const samples: PathSample[] = [];
  for (let i = 0; i <= steps; i++) {
    const base = (i * sampleStride) / 4;
    samples.push({
      position: [
        view[base + 0] ?? 0,
        view[base + 1] ?? 0,
        view[base + 2] ?? 0,
        view[base + 3] ?? 0,
      ],
      velocity: [
        view[base + 4] ?? 0,
        view[base + 5] ?? 0,
        view[base + 6] ?? 0,
        view[base + 7] ?? 0,
      ],
      tau: view[base + 8] ?? 0,
      ds2: view[base + 9] ?? 0,
    });
  }
  return { samples, millis: t1 - t0 };
}

function draw(samples: PathSample[], key: SignatureKey, gpuPath: PathSample[] | null): void {
  const ctx = canvas.getContext("2d");
  if (!ctx) return;
  ctx.fillStyle = "#15171c";
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  const eta = canonicalDiagonal(SIGNATURES[key]);
  ctx.strokeStyle = "#3a3d44";
  ctx.beginPath();
  ctx.moveTo(0, canvas.height / 2);
  ctx.lineTo(canvas.width, canvas.height / 2);
  ctx.moveTo(canvas.width / 2, 0);
  ctx.lineTo(canvas.width / 2, canvas.height);
  ctx.stroke();

  ctx.font = "12px ui-monospace, monospace";
  ctx.fillStyle = "#8a8f99";
  ctx.fillText(`signature (p,q,r) = (${SIGNATURES[key].p}, ${SIGNATURES[key].q}, ${SIGNATURES[key].r})`, 12, 18);
  ctx.fillText(`η diag = [${[...eta].map((v) => v.toFixed(0)).join(", ")}]`, 12, 36);

  const cx = canvas.width / 2;
  const cy = canvas.height / 2;
  const scale = 30;

  if (gpuPath !== null) {
    ctx.lineWidth = 4;
    ctx.strokeStyle = "rgba(124, 196, 255, 0.35)";
    ctx.beginPath();
    for (let i = 0; i < gpuPath.length; i++) {
      const p = gpuPath[i]!.position;
      const x = cx + (p[0] ?? 0) * scale;
      const y = cy - (p[3] ?? 0) * scale;
      if (i === 0) ctx.moveTo(x, y);
      else ctx.lineTo(x, y);
    }
    ctx.stroke();
  }

  ctx.lineWidth = 1.5;
  ctx.strokeStyle = "#7cc4ff";
  ctx.beginPath();
  for (let i = 0; i < samples.length; i++) {
    const p = samples[i]!.position;
    const x = cx + (p[0] ?? 0) * scale;
    const y = cy - (p[3] ?? 0) * scale;
    if (i === 0) ctx.moveTo(x, y);
    else ctx.lineTo(x, y);
  }
  ctx.stroke();

  const last = samples[samples.length - 1]!;
  ctx.fillStyle = "#e6e8ea";
  ctx.fillText(
    `τ_end = ${last.tau.toFixed(3)}  ds²_end = ${last.ds2.toFixed(4)}  (sign = ${Math.sign(last.ds2).toFixed(0)})`,
    12,
    canvas.height - 14,
  );
}

function endpointDelta(cpu: PathSample[], gpuPath: PathSample[]): number {
  const a = cpu[cpu.length - 1]!.position;
  const b = gpuPath[gpuPath.length - 1]!.position;
  let s = 0;
  for (let i = 0; i < 4; i++) s += ((a[i] ?? 0) - (b[i] ?? 0)) ** 2;
  return Math.sqrt(s);
}

async function reintegrate(): Promise<void> {
  const key = currentSignature();
  const sig = SIGNATURES[key];
  const v0 = Number(v0Input.value);
  const tauMax = Number(taumaxInput.value);
  const steps = Math.max(1, Number(stepsInput.value));
  const v0Vec = [v0, 0, 0, 1];
  const x0Vec = [0, 0, 0, 0];
  const t0 = performance.now();
  const path = integrateFlat(sig, x0Vec, v0Vec, { steps, tauMax });
  const t1 = performance.now();
  let gpuPath: PathSample[] | null = null;
  let gpuMs = -1;
  if (gpu) {
    try {
      const run = await runWebGPU(sig, v0Vec, tauMax, steps);
      if (run !== null) {
        gpuPath = run.samples;
        gpuMs = run.millis;
      }
    } catch (e) {
      log(`GPU path failed: ${e instanceof Error ? e.message : String(e)}`);
    }
  }
  draw(path, key, gpuPath);
  let line =
    `integrated ${steps} steps (${key}, η=[${[...canonicalDiagonal(sig)].map((v) => v.toFixed(0)).join(", ")}]) in ${(t1 - t0).toFixed(2)} ms (CPU)`;
  if (gpuPath !== null) {
    const delta = endpointDelta(path, gpuPath);
    line += `; GPU ${gpuMs.toFixed(2)} ms; CPU/GPU endpoint Δ = ${delta.toExponential(2)}`;
  }
  log(line);
}

async function bootstrap(): Promise<void> {
  await initWebGPU();
  runBtn.addEventListener("click", () => {
    void reintegrate();
  });
  document.querySelectorAll<HTMLInputElement>('input[name="sig"]').forEach((el) => {
    el.addEventListener("change", () => {
      void reintegrate();
    });
  });
  void reintegrate();
}

bootstrap().catch((e: unknown) => log(`bootstrap error: ${e instanceof Error ? e.message : String(e)}`));
