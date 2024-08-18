#[doc = "Register `INTERP1_BASE0` reader"]
pub type R = crate::R<INTERP1_BASE0_SPEC>;
#[doc = "Register `INTERP1_BASE0` writer"]
pub type W = crate::W<INTERP1_BASE0_SPEC>;
#[doc = "Field `INTERP1_BASE0` reader - "]
pub type INTERP1_BASE0_R = crate::FieldReader<u32>;
#[doc = "Field `INTERP1_BASE0` writer - "]
pub type INTERP1_BASE0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn interp1_base0(&self) -> INTERP1_BASE0_R {
        INTERP1_BASE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn interp1_base0(&mut self) -> INTERP1_BASE0_W<INTERP1_BASE0_SPEC> {
        INTERP1_BASE0_W::new(self, 0)
    }
}
#[doc = "Read/write access to BASE0 register.  

You can [`read`](crate::Reg::read) this register and get [`interp1_base0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interp1_base0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERP1_BASE0_SPEC;
impl crate::RegisterSpec for INTERP1_BASE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interp1_base0::R`](R) reader structure"]
impl crate::Readable for INTERP1_BASE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interp1_base0::W`](W) writer structure"]
impl crate::Writable for INTERP1_BASE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERP1_BASE0 to value 0"]
impl crate::Resettable for INTERP1_BASE0_SPEC {
    const RESET_VALUE: u32 = 0;
}