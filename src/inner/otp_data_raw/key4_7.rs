#[doc = "Register `KEY4_7` reader"]
pub type R = crate::R<KEY4_7_SPEC>;
#[doc = "Register `KEY4_7` writer"]
pub type W = crate::W<KEY4_7_SPEC>;
#[doc = "Field `KEY4_7` reader - "]
pub type KEY4_7_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn key4_7(&self) -> KEY4_7_R {
        KEY4_7_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Bits 127:112 of OTP access key 4 (ECC)  

You can [`read`](crate::Reg::read) this register and get [`key4_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`key4_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY4_7_SPEC;
impl crate::RegisterSpec for KEY4_7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key4_7::R`](R) reader structure"]
impl crate::Readable for KEY4_7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key4_7::W`](W) writer structure"]
impl crate::Writable for KEY4_7_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY4_7 to value 0"]
impl crate::Resettable for KEY4_7_SPEC {
    const RESET_VALUE: u32 = 0;
}
