#[doc = "Register `FP_PIDR0` reader"]
pub type R = crate::R<FP_PIDR0_SPEC>;
#[doc = "Register `FP_PIDR0` writer"]
pub type W = crate::W<FP_PIDR0_SPEC>;
#[doc = "Field `PART_0` reader - See CoreSight Architecture Specification"]
pub type PART_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - See CoreSight Architecture Specification"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {}
#[doc = "Provides CoreSight discovery information for the FP  

You can [`read`](crate::Reg::read) this register and get [`fp_pidr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fp_pidr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FP_PIDR0_SPEC;
impl crate::RegisterSpec for FP_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fp_pidr0::R`](R) reader structure"]
impl crate::Readable for FP_PIDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fp_pidr0::W`](W) writer structure"]
impl crate::Writable for FP_PIDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FP_PIDR0 to value 0x21"]
impl crate::Resettable for FP_PIDR0_SPEC {
    const RESET_VALUE: u32 = 0x21;
}