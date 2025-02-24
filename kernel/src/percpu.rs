use crate::cpu_set::LogicalCpuId;

/// The percpu block, that stored all percpu variable.
pub struct PercpuBlock {
    pub cpu_id: LogicalCpuId,
}
