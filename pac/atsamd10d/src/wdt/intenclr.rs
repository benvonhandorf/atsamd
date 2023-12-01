# [doc = "Register `INTENCLR` reader"] pub type R = crate :: R < INTENCLR_SPEC > ; # [doc = "Register `INTENCLR` writer"] pub type W = crate :: W < INTENCLR_SPEC > ; # [doc = "Field `EW` reader - Early Warning Interrupt Enable"] pub type EW_R = crate :: BitReader ; # [doc = "Field `EW` writer - Early Warning Interrupt Enable"] pub type EW_W < 'a , REG > = crate :: BitWriter < 'a , REG > ; impl R { # [doc = "Bit 0 - Early Warning Interrupt Enable"] # [inline (always)] pub fn ew (& self) -> EW_R { EW_R :: new ((self . bits & 1) != 0) } } impl W { # [doc = "Bit 0 - Early Warning Interrupt Enable"] # [inline (always)] # [must_use] pub fn ew (& mut self) -> EW_W < INTENCLR_SPEC > { EW_W :: new (self , 0) } # [doc = r" Writes raw bits to the register."] # [doc = r""] # [doc = r" # Safety"] # [doc = r""] # [doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"] # [inline (always)] pub unsafe fn bits (& mut self , bits : u8) -> & mut Self { self . bits = bits ; self } } # [doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intenclr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."] pub struct INTENCLR_SPEC ; impl crate :: RegisterSpec for INTENCLR_SPEC { type Ux = u8 ; } # [doc = "`read()` method returns [`intenclr::R`](R) reader structure"] impl crate :: Readable for INTENCLR_SPEC { } # [doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"] impl crate :: Writable for INTENCLR_SPEC { const ZERO_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; const ONE_TO_MODIFY_FIELDS_BITMAP : Self :: Ux = 0 ; } # [doc = "`reset()` method sets INTENCLR to value 0"] impl crate :: Resettable for INTENCLR_SPEC { const RESET_VALUE : Self :: Ux = 0 ; }