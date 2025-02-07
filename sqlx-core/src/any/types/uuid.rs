use crate::any::{Any, AnyTypeInfo, AnyTypeInfoKind, AnyValueKind};
use crate::database::Database;
use crate::decode::Decode;
use crate::encode::{Encode, IsNull};
use crate::error::BoxDynError;
use crate::types::Type;
use uuid::{
    fmt::{Hyphenated, Simple},
    Uuid,
};

impl Type<Any> for Uuid {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Uuid,
        }
    }

    fn compatible(ty: &AnyTypeInfo) -> bool {
        matches!(ty.kind, AnyTypeInfoKind::Blob | AnyTypeInfoKind::Text)
    }
}

impl<'q> Encode<'q, Any> for Uuid {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        buf.0.push(AnyValueKind::Uuid(self.into_uuid()));

        Ok(IsNull::No)
    }
}

impl<'a> Decode<'a, Any> for Uuid {
    fn decode(value: <Any as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Uuid(u) => Uuid::from_slice(u.as_bytes()).map_err(Into::into),
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for Hyphenated {
    fn type_info() -> AnyTypeInfo {
        AnyTypeInfo {
            kind: AnyTypeInfoKind::Uuid,
        }
    }
}

impl<'q> Encode<'q, Any> for Hyphenated {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        buf.0.push(AnyValueKind::Uuid(self.into_uuid()));

        Ok(IsNull::No)
    }
}

impl<'a> Decode<'a, Any> for Hyphenated {
    fn decode(value: <Any as Database>::ValueRef<'a>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Uuid(u) => {
                let uuid = Uuid::parse_str(&u.map(ToOwned::to_owned)?).map_err(Into::into);
                Ok(uuid?.hyphenated())
            }
            other => other.unexpected(),
        }
    }
}

impl Type<Any> for Simple {
    fn type_info() -> AnyTypeInfo {
        <crate::any::types::uuid as Type<Any>>::type_info()
    }
}

impl<'q> Encode<'q, Any> for Simple {
    fn encode_by_ref(
        &self,
        buf: &mut <Any as Database>::ArgumentBuffer<'q>,
    ) -> Result<IsNull, BoxDynError> {
        buf.0.push(AnyValueKind::Uuid(self.into_uuid()));

        Ok(IsNull::No)
    }
}

impl<'r> Decode<'r, Any> for Simple {
    fn decode(value: <Any as Database>::ValueRef<'r>) -> Result<Self, BoxDynError> {
        match value.kind {
            AnyValueKind::Uuid(u) => {
                let uuid = Uuid::parse_str(&u.map(ToOwned::to_owned)?).map_err(Into::into);
                Ok(uuid?.simple())
            }
            other => other.unexpected(),
        }
    }
}
