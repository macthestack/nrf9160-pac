#[doc = "RAM address pointer to write samples to with EasyDMA\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [ptr](ptr) module"]
pub type PTR = crate::Reg<u32, _PTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PTR;
#[doc = "`read()` method returns [ptr::R](ptr::R) reader structure"]
impl crate::Readable for PTR {}
#[doc = "`write(|w| ..)` method takes [ptr::W](ptr::W) writer structure"]
impl crate::Writable for PTR {}
#[doc = "RAM address pointer to write samples to with EasyDMA"]
pub mod ptr;
#[doc = "Number of samples to allocate memory for in EasyDMA mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about avaliable fields see [maxcnt](maxcnt) module"]
pub type MAXCNT = crate::Reg<u32, _MAXCNT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MAXCNT;
#[doc = "`read()` method returns [maxcnt::R](maxcnt::R) reader structure"]
impl crate::Readable for MAXCNT {}
#[doc = "`write(|w| ..)` method takes [maxcnt::W](maxcnt::W) writer structure"]
impl crate::Writable for MAXCNT {}
#[doc = "Number of samples to allocate memory for in EasyDMA mode"]
pub mod maxcnt;
