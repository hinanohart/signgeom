// signgeom WGSL — geodesic step kernel for flat manifolds of arbitrary
// signature. The kernel writes one segment per (signature, sample) and is
// intentionally simple: flat metric => zero Christoffel => straight line.
// Same kernel works for (4,0,0), (3,1,0), (2,2,0) by toggling the eta sign.

struct Params {
  // η_{ii} signs (only used by the renderer to colour worldlines).
  eta: vec4<f32>,
  // initial velocity v^μ
  v0: vec4<f32>,
  // number of samples requested by the host (≤ 4096)
  steps: u32,
  // total proper time
  tau_max: f32,
  // padding for 16-byte alignment
  _pad0: u32,
  _pad1: u32,
};

@group(0) @binding(0) var<uniform> params: Params;

struct Sample {
  position: vec4<f32>,
  velocity: vec4<f32>,
  tau: f32,
  ds2: f32,
  _pad0: f32,
  _pad1: f32,
};

@group(0) @binding(1) var<storage, read_write> samples: array<Sample>;

@compute @workgroup_size(64)
fn integrate(@builtin(global_invocation_id) gid: vec3<u32>) {
  let i = gid.x;
  if (i > params.steps) { return; }
  let dt = params.tau_max / f32(params.steps);
  let tau = f32(i) * dt;
  let pos = params.v0 * tau;
  // line element ds² = η_{kk} (v^k)² τ — signed in pseudo-Riemannian sigs.
  let v2 = params.v0 * params.v0;
  let line = dot(params.eta, v2);
  let ds2 = line * tau;
  var s: Sample;
  s.position = pos;
  s.velocity = params.v0;
  s.tau = tau;
  s.ds2 = ds2;
  s._pad0 = 0.0;
  s._pad1 = 0.0;
  samples[i] = s;
}
