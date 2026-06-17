use waclay::*;

use crate::wasip2::{ResultType, WasiP2Resources};

#[derive(Default, Debug)]
pub struct WasiP2TypesBuilder {
    pub stream_error: ComputableType,
    pub error_code: ComputableType,
    pub descriptor_flags: ComputableType,
    pub descriptor_type: ComputableType,
    pub descriptor_stat: ComputableType,
    pub datetime: ComputableType,
    pub path_flags: ComputableType,
    pub metadata_hash_value: ComputableType,
}

#[allow(unused)]
pub struct WasiP2Types {
    pub stream_error: ValueType,
    pub error_code: ValueType,
    pub descriptor_flags: ValueType,
    pub descriptor_type: ValueType,
    pub descriptor_stat: ValueType,
    pub datetime: ValueType,
    pub path_flags: ValueType,
    pub metadata_hash_value: ValueType,
}

impl WasiP2TypesBuilder {
    pub fn get_type(
        &mut self,
        resources: &WasiP2Resources,
        _name: &str,
        mut selector: impl FnMut(&mut Self) -> &mut ComputableType,
    ) -> ResultType<ValueType> {
        match selector(self).get_value()? {
            GetValueResult::Compute(compute) => {
                let value = compute(self, resources)?;
                let comp = selector(self); // TODO: re-running selector, not great
                *comp = ComputableType::Resolved(value.clone());
                Ok(value)
            }
            GetValueResult::Resolved(value) => Ok(value),
        }
    }

    pub fn build(mut self, resources: &WasiP2Resources) -> ResultType<WasiP2Types> {
        let stream_error = self.get_type(resources, "stream_error", |b| &mut b.stream_error)?;
        let error_code = self.get_type(resources, "error_code", |b| &mut b.error_code)?;
        let descriptor_flags =
            self.get_type(resources, "descriptor_flags", |b| &mut b.descriptor_flags)?;
        let descriptor_type =
            self.get_type(resources, "descriptor_type", |b| &mut b.descriptor_type)?;
        let descriptor_stat =
            self.get_type(resources, "descriptor_stat", |b| &mut b.descriptor_stat)?;
        let datetime = self.get_type(resources, "datetime", |b| &mut b.datetime)?;
        let path_flags = self.get_type(resources, "path_flags", |b| &mut b.path_flags)?;
        let metadata_hash_value = self.get_type(resources, "metadata_hash_value", |b| {
            &mut b.metadata_hash_value
        })?;

        Ok(WasiP2Types {
            stream_error,
            error_code,
            descriptor_flags,
            descriptor_type,
            descriptor_stat,
            datetime,
            path_flags,
            metadata_hash_value,
        })
    }
}

pub enum ComputableType {
    Uninitialized,
    Ready(Box<dyn FnOnce(&mut WasiP2TypesBuilder, &WasiP2Resources) -> ResultType<ValueType>>),
    Computing,
    Resolved(ValueType),
}

impl ComputableType {
    pub fn set_fn(
        &mut self,
        compute: impl FnOnce(&mut WasiP2TypesBuilder, &WasiP2Resources) -> ResultType<ValueType>
            + 'static,
    ) -> ResultType<()> {
        if !matches!(self, ComputableType::Uninitialized) {
            panic!("TODO: Cannot set the fn of ComputableType more than once");
        }
        *self = Self::Ready(Box::new(compute));
        Ok(())
    }

    pub fn get_value(&mut self) -> ResultType<GetValueResult> {
        let old_value = std::mem::replace(self, Self::Computing);
        match old_value {
            Self::Uninitialized => {
                *self = Self::Uninitialized;
                panic!("TODO: ValueType is Uninitialized")
            }
            Self::Ready(compute) => Ok(GetValueResult::Compute(compute)),
            Self::Computing => {
                panic!("TODO: recursion detected in type definition");
            }
            Self::Resolved(ty) => {
                *self = Self::Resolved(ty.clone());
                Ok(GetValueResult::Resolved(ty))
            }
        }
    }
}

pub enum GetValueResult {
    Resolved(ValueType),
    Compute(Box<dyn FnOnce(&mut WasiP2TypesBuilder, &WasiP2Resources) -> ResultType<ValueType>>),
}

impl Default for ComputableType {
    fn default() -> Self {
        Self::Uninitialized
    }
}

impl std::fmt::Debug for ComputableType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uninitialized => write!(f, "Uninitialized"),
            Self::Ready(_) => write!(f, "Ready"),
            Self::Computing => write!(f, "Computing"),
            Self::Resolved(_) => write!(f, "Resolved"),
        }
    }
}
