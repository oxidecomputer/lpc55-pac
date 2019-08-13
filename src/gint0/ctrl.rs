#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTRL {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `INT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum INTR {
    #[doc = "No request. No interrupt request is pending."]
    NO_REQUEST,
    #[doc = "Request active. Interrupt request is active."]
    REQUEST_ACTIVE,
}
impl INTR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            INTR::NO_REQUEST => false,
            INTR::REQUEST_ACTIVE => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> INTR {
        match value {
            false => INTR::NO_REQUEST,
            true => INTR::REQUEST_ACTIVE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_REQUEST`"]
    #[inline]
    pub fn is_no_request(&self) -> bool {
        *self == INTR::NO_REQUEST
    }
    #[doc = "Checks if the value of the field is `REQUEST_ACTIVE`"]
    #[inline]
    pub fn is_request_active(&self) -> bool {
        *self == INTR::REQUEST_ACTIVE
    }
}
#[doc = "Possible values of the field `COMB`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum COMBR {
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    OR,
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    AND,
}
impl COMBR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            COMBR::OR => false,
            COMBR::AND => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> COMBR {
        match value {
            false => COMBR::OR,
            true => COMBR::AND,
        }
    }
    #[doc = "Checks if the value of the field is `OR`"]
    #[inline]
    pub fn is_or(&self) -> bool {
        *self == COMBR::OR
    }
    #[doc = "Checks if the value of the field is `AND`"]
    #[inline]
    pub fn is_and(&self) -> bool {
        *self == COMBR::AND
    }
}
#[doc = "Possible values of the field `TRIG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TRIGR {
    #[doc = "Edge-triggered."]
    EDGE_TRIGGERED,
    #[doc = "Level-triggered."]
    LEVEL_TRIGGERED,
}
impl TRIGR {
    #[doc = r" Returns `true` if the bit is clear (0)"]
    #[inline]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r" Returns `true` if the bit is set (1)"]
    #[inline]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bit(&self) -> bool {
        match *self {
            TRIGR::EDGE_TRIGGERED => false,
            TRIGR::LEVEL_TRIGGERED => true,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: bool) -> TRIGR {
        match value {
            false => TRIGR::EDGE_TRIGGERED,
            true => TRIGR::LEVEL_TRIGGERED,
        }
    }
    #[doc = "Checks if the value of the field is `EDGE_TRIGGERED`"]
    #[inline]
    pub fn is_edge_triggered(&self) -> bool {
        *self == TRIGR::EDGE_TRIGGERED
    }
    #[doc = "Checks if the value of the field is `LEVEL_TRIGGERED`"]
    #[inline]
    pub fn is_level_triggered(&self) -> bool {
        *self == TRIGR::LEVEL_TRIGGERED
    }
}
#[doc = "Values that can be written to the field `INT`"]
pub enum INTW {
    #[doc = "No request. No interrupt request is pending."]
    NO_REQUEST,
    #[doc = "Request active. Interrupt request is active."]
    REQUEST_ACTIVE,
}
impl INTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            INTW::NO_REQUEST => false,
            INTW::REQUEST_ACTIVE => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _INTW<'a> {
    w: &'a mut W,
}
impl<'a> _INTW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: INTW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No request. No interrupt request is pending."]
    #[inline]
    pub fn no_request(self) -> &'a mut W {
        self.variant(INTW::NO_REQUEST)
    }
    #[doc = "Request active. Interrupt request is active."]
    #[inline]
    pub fn request_active(self) -> &'a mut W {
        self.variant(INTW::REQUEST_ACTIVE)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `COMB`"]
pub enum COMBW {
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    OR,
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    AND,
}
impl COMBW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            COMBW::OR => false,
            COMBW::AND => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _COMBW<'a> {
    w: &'a mut W,
}
impl<'a> _COMBW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: COMBW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Or. OR functionality: A grouped interrupt is generated when any one of the enabled inputs is active (based on its programmed polarity)."]
    #[inline]
    pub fn or(self) -> &'a mut W {
        self.variant(COMBW::OR)
    }
    #[doc = "And. AND functionality: An interrupt is generated when all enabled bits are active (based on their programmed polarity)."]
    #[inline]
    pub fn and(self) -> &'a mut W {
        self.variant(COMBW::AND)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `TRIG`"]
pub enum TRIGW {
    #[doc = "Edge-triggered."]
    EDGE_TRIGGERED,
    #[doc = "Level-triggered."]
    LEVEL_TRIGGERED,
}
impl TRIGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            TRIGW::EDGE_TRIGGERED => false,
            TRIGW::LEVEL_TRIGGERED => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _TRIGW<'a> {
    w: &'a mut W,
}
impl<'a> _TRIGW<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: TRIGW) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "Edge-triggered."]
    #[inline]
    pub fn edge_triggered(self) -> &'a mut W {
        self.variant(TRIGW::EDGE_TRIGGERED)
    }
    #[doc = "Level-triggered."]
    #[inline]
    pub fn level_triggered(self) -> &'a mut W {
        self.variant(TRIGW::LEVEL_TRIGGERED)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline]
    pub fn int(&self) -> INTR {
        INTR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline]
    pub fn comb(&self) -> COMBR {
        COMBR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 1;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline]
    pub fn trig(&self) -> TRIGR {
        TRIGR::_from({
            const MASK: bool = true;
            const OFFSET: u8 = 2;
            ((self.bits >> OFFSET) & MASK as u32) != 0
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Group interrupt status. This bit is cleared by writing a one to it. Writing zero has no effect."]
    #[inline]
    pub fn int(&mut self) -> _INTW {
        _INTW { w: self }
    }
    #[doc = "Bit 1 - Combine enabled inputs for group interrupt"]
    #[inline]
    pub fn comb(&mut self) -> _COMBW {
        _COMBW { w: self }
    }
    #[doc = "Bit 2 - Group interrupt trigger"]
    #[inline]
    pub fn trig(&mut self) -> _TRIGW {
        _TRIGW { w: self }
    }
}