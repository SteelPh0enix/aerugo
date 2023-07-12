#[doc = "Register `PIDR5` reader"]
pub struct R(crate::R<PIDR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIDR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIDR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIDR5_SPEC>) -> Self {
        R(reader)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PIDR5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "ETM Peripheral Identification Register #5\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pidr5](index.html) module"]
pub struct PIDR5_SPEC;
impl crate::RegisterSpec for PIDR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pidr5::R](R) reader structure"]
impl crate::Readable for PIDR5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PIDR5 to value 0"]
impl crate::Resettable for PIDR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
