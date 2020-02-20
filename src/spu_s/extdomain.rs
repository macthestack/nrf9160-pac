#[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perm](perm) module"]
pub type PERM = crate::Reg<u32, _PERM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERM;
#[doc = "`read()` method returns [perm::R](perm::R) reader structure"]
impl crate::Readable for PERM {}
#[doc = "`write(|w| ..)` method takes [perm::W](perm::W) writer structure"]
impl crate::Writable for PERM {}
#[doc = "Description cluster: Access for bus access generated from the external domain n List capabilities of the external domain n"]
pub mod perm;
