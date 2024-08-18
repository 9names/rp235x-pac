#[doc = "Register `INTERP0_POP_FULL` reader"]
pub type R = crate::R<INTERP0_POP_FULL_SPEC>;
#[doc = "Register `INTERP0_POP_FULL` writer"]
pub type W = crate::W<INTERP0_POP_FULL_SPEC>;
#[doc = "Field `INTERP0_POP_FULL` reader - "]
pub type INTERP0_POP_FULL_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp0_pop_full(&self) -> INTERP0_POP_FULL_R {
        INTERP0_POP_FULL_R::new(self.bits)
    }
}
impl W {}
#[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP).  

You can [`read`](crate::Reg::read) this register and get [`interp0_pop_full::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp0_pop_full::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP0_POP_FULL_SPEC;
impl crate::RegisterSpec for INTERP0_POP_FULL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp0_pop_full::R`](R) reader structure"]
impl crate::Readable for INTERP0_POP_FULL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp0_pop_full::W`](W) writer structure"]
impl crate::Writable for INTERP0_POP_FULL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP0_POP_FULL to value 0"]
impl crate::Resettable for INTERP0_POP_FULL_SPEC {
    const RESET_VALUE: u32 = 0;
}