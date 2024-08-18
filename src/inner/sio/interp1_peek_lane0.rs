#[doc = "Register `INTERP1_PEEK_LANE0` reader"]
pub type R = crate::R<INTERP1_PEEK_LANE0_SPEC>;
#[doc = "Register `INTERP1_PEEK_LANE0` writer"]
pub type W = crate::W<INTERP1_PEEK_LANE0_SPEC>;
#[doc = "Field `INTERP1_PEEK_LANE0` reader - "]
pub type INTERP1_PEEK_LANE0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp1_peek_lane0(&self) -> INTERP1_PEEK_LANE0_R {
        INTERP1_PEEK_LANE0_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read LANE0 result, without altering any internal state (PEEK).  

You can [`read`](crate::Reg::read) this register and get [`interp1_peek_lane0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_peek_lane0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_PEEK_LANE0_SPEC;
impl crate::RegisterSpec for INTERP1_PEEK_LANE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_peek_lane0::R`](R) reader structure"]
impl crate::Readable for INTERP1_PEEK_LANE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp1_peek_lane0::W`](W) writer structure"]
impl crate::Writable for INTERP1_PEEK_LANE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP1_PEEK_LANE0 to value 0"]
impl crate::Resettable for INTERP1_PEEK_LANE0_SPEC {
    const RESET_VALUE: u32 = 0;
}