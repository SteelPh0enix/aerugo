#[doc = "Register `OWDR` writer"]
pub struct W(crate::W<OWDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OWDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<OWDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OWDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P0` writer - Output Write Disable"]
pub type P0_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P1` writer - Output Write Disable"]
pub type P1_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P2` writer - Output Write Disable"]
pub type P2_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P3` writer - Output Write Disable"]
pub type P3_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P4` writer - Output Write Disable"]
pub type P4_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P5` writer - Output Write Disable"]
pub type P5_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P6` writer - Output Write Disable"]
pub type P6_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P7` writer - Output Write Disable"]
pub type P7_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P8` writer - Output Write Disable"]
pub type P8_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P9` writer - Output Write Disable"]
pub type P9_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P10` writer - Output Write Disable"]
pub type P10_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P11` writer - Output Write Disable"]
pub type P11_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P12` writer - Output Write Disable"]
pub type P12_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P13` writer - Output Write Disable"]
pub type P13_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P14` writer - Output Write Disable"]
pub type P14_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P15` writer - Output Write Disable"]
pub type P15_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P16` writer - Output Write Disable"]
pub type P16_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P17` writer - Output Write Disable"]
pub type P17_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P18` writer - Output Write Disable"]
pub type P18_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P19` writer - Output Write Disable"]
pub type P19_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P20` writer - Output Write Disable"]
pub type P20_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P21` writer - Output Write Disable"]
pub type P21_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P22` writer - Output Write Disable"]
pub type P22_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P23` writer - Output Write Disable"]
pub type P23_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P24` writer - Output Write Disable"]
pub type P24_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P25` writer - Output Write Disable"]
pub type P25_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P26` writer - Output Write Disable"]
pub type P26_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P27` writer - Output Write Disable"]
pub type P27_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P28` writer - Output Write Disable"]
pub type P28_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P29` writer - Output Write Disable"]
pub type P29_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P30` writer - Output Write Disable"]
pub type P30_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
#[doc = "Field `P31` writer - Output Write Disable"]
pub type P31_W<'a, const O: u8> = crate::BitWriter<'a, OWDR_SPEC, O>;
impl W {
    #[doc = "Bit 0 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p0(&mut self) -> P0_W<0> {
        P0_W::new(self)
    }
    #[doc = "Bit 1 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p1(&mut self) -> P1_W<1> {
        P1_W::new(self)
    }
    #[doc = "Bit 2 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p2(&mut self) -> P2_W<2> {
        P2_W::new(self)
    }
    #[doc = "Bit 3 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p3(&mut self) -> P3_W<3> {
        P3_W::new(self)
    }
    #[doc = "Bit 4 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p4(&mut self) -> P4_W<4> {
        P4_W::new(self)
    }
    #[doc = "Bit 5 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p5(&mut self) -> P5_W<5> {
        P5_W::new(self)
    }
    #[doc = "Bit 6 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p6(&mut self) -> P6_W<6> {
        P6_W::new(self)
    }
    #[doc = "Bit 7 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p7(&mut self) -> P7_W<7> {
        P7_W::new(self)
    }
    #[doc = "Bit 8 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p8(&mut self) -> P8_W<8> {
        P8_W::new(self)
    }
    #[doc = "Bit 9 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p9(&mut self) -> P9_W<9> {
        P9_W::new(self)
    }
    #[doc = "Bit 10 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p10(&mut self) -> P10_W<10> {
        P10_W::new(self)
    }
    #[doc = "Bit 11 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p11(&mut self) -> P11_W<11> {
        P11_W::new(self)
    }
    #[doc = "Bit 12 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p12(&mut self) -> P12_W<12> {
        P12_W::new(self)
    }
    #[doc = "Bit 13 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p13(&mut self) -> P13_W<13> {
        P13_W::new(self)
    }
    #[doc = "Bit 14 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p14(&mut self) -> P14_W<14> {
        P14_W::new(self)
    }
    #[doc = "Bit 15 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p15(&mut self) -> P15_W<15> {
        P15_W::new(self)
    }
    #[doc = "Bit 16 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p16(&mut self) -> P16_W<16> {
        P16_W::new(self)
    }
    #[doc = "Bit 17 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p17(&mut self) -> P17_W<17> {
        P17_W::new(self)
    }
    #[doc = "Bit 18 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p18(&mut self) -> P18_W<18> {
        P18_W::new(self)
    }
    #[doc = "Bit 19 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p19(&mut self) -> P19_W<19> {
        P19_W::new(self)
    }
    #[doc = "Bit 20 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<20> {
        P20_W::new(self)
    }
    #[doc = "Bit 21 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<21> {
        P21_W::new(self)
    }
    #[doc = "Bit 22 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<22> {
        P22_W::new(self)
    }
    #[doc = "Bit 23 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<23> {
        P23_W::new(self)
    }
    #[doc = "Bit 24 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<24> {
        P24_W::new(self)
    }
    #[doc = "Bit 25 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<25> {
        P25_W::new(self)
    }
    #[doc = "Bit 26 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<26> {
        P26_W::new(self)
    }
    #[doc = "Bit 27 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<27> {
        P27_W::new(self)
    }
    #[doc = "Bit 28 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<28> {
        P28_W::new(self)
    }
    #[doc = "Bit 29 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<29> {
        P29_W::new(self)
    }
    #[doc = "Bit 30 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p30(&mut self) -> P30_W<30> {
        P30_W::new(self)
    }
    #[doc = "Bit 31 - Output Write Disable"]
    #[inline(always)]
    #[must_use]
    pub fn p31(&mut self) -> P31_W<31> {
        P31_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Output Write Disable\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [owdr](index.html) module"]
pub struct OWDR_SPEC;
impl crate::RegisterSpec for OWDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [owdr::W](W) writer structure"]
impl crate::Writable for OWDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OWDR to value 0"]
impl crate::Resettable for OWDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
