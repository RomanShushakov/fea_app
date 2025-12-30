#[derive(Clone, Copy, Debug)]
pub enum ResultPlot
{
    Displacements(f32),
    GlobalForces,
    GlobalMoments,
    ForceR,
    ForceS,
    ForceT,
    MomentR,
    MomentS,
    MomentT,
    MemForceR,
    MemForceS,
    MemForceRS,
    BendMomentR,
    BendMomentS,
    BendMomentRS,
    ShearForceRT,
    ShearForceST,
}
