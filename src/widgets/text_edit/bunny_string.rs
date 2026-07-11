use std::{
    borrow::{Borrow, Cow},
    fmt::{self, Display},
    ops::{Deref, Index},
    str::FromStr,
};

use abi_stable::{
    reexports::SelfOps,
    std_types::{
        RCowStr, RStr, RString,
        string::{Drain, IntoIter},
    },
};
use egui::{TextBuffer, text_selection::text_cursor_state::byte_index_from_char_index};
use serde::{Deserialize, Serialize};

macro_rules! deref_coerced_impl_cmp_traits {
    (
        $Self:ty;
        coerce_to = $coerce_to:ty,
        [$($Rhs:ty),* $(,)?]
    ) => {
        const _: () = {
            use std::cmp::{PartialEq, PartialOrd, Ordering};

            $(

                impl PartialEq<$Rhs> for $Self {
                    fn eq(&self, other: &$Rhs) -> bool {
                        <$coerce_to as PartialEq>::eq(self, other)
                    }
                }

                impl PartialOrd<$Rhs> for $Self {
                    fn partial_cmp(&self, other: &$Rhs) -> Option<Ordering> {
                        <$coerce_to as PartialOrd>::partial_cmp(self, other)
                    }
                }

                impl PartialEq<$Self> for $Rhs {
                    fn eq(&self, other: &$Self) -> bool {
                        <$coerce_to as PartialEq>::eq(self, other)
                    }
                }

                impl PartialOrd<$Self> for $Rhs {
                    fn partial_cmp(&self, other: &$Self) -> Option<Ordering> {
                        <$coerce_to as PartialOrd>::partial_cmp(self, other)
                    }
                }
            )*
        };
    };
}

/// An FFI safe string type that implements egui::widgets::text_edit::TextBuffer.
/// For use with bunny_ui::widgets::TextEdit
#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BunnyString(RString);

impl BunnyString {
    pub const fn new() -> Self {
        Self(RString::new())
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self(RString::with_capacity(cap))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub const fn capacity(&self) -> usize {
        self.0.capacity()
    }

    pub fn into_string(self) -> String {
        self.0.into_string()
    }

    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    pub fn shrink_to_fit(&mut self) {
        self.0.shrink_to_fit();
    }

    pub fn reserve_exact(&mut self, additional: usize) {
        self.0.reserve_exact(additional);
    }

    pub fn push(&mut self, ch: char) {
        self.0.push(ch);
    }

    pub fn push_str(&mut self, str: &str) {
        self.0.push_str(str);
    }

    pub fn pop(&mut self) -> Option<char> {
        self.0.pop()
    }

    pub fn remove(&mut self, idx: usize) -> char {
        self.0.remove(idx)
    }

    pub fn insert(&mut self, idx: usize, ch: char) {
        self.0.insert(idx, ch);
    }

    pub fn insert_str(&mut self, idx: usize, string: &str) {
        self.0.insert_str(idx, string);
    }

    #[inline]
    pub fn retain<F>(&mut self, pred: F)
    where
        F: FnMut(char) -> bool,
    {
        self.0.retain(pred);
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn drain<I>(&mut self, range: I) -> Drain<'_>
    where
        str: Index<I, Output = str>,
    {
        self.0.drain(range)
    }
}

impl Default for BunnyString {
    fn default() -> Self {
        Self::new()
    }
}

deref_coerced_impl_cmp_traits! {
    BunnyString;
    coerce_to = str,
    [
        String,
        str,
        &str,
        RStr<'_>,
        std::borrow::Cow<'_, str>,
        RCowStr<'_>,
        RString,
    ]
}

impl From<String> for BunnyString {
    fn from(value: String) -> Self {
        Self(value.into())
    }
}

impl From<BunnyString> for String {
    fn from(value: BunnyString) -> Self {
        value.0.into_string()
    }
}

impl From<RString> for BunnyString {
    fn from(value: RString) -> Self {
        Self(value)
    }
}

impl From<BunnyString> for RString {
    fn from(value: BunnyString) -> Self {
        value.0
    }
}

impl FromStr for BunnyString {
    type Err = <String as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.parse::<String>().map(RString::from)?;
        Ok(Self(s))
    }
}

impl From<&str> for BunnyString {
    fn from(value: &str) -> Self {
        Self(value.into())
    }
}

impl<'a> From<Cow<'a, str>> for BunnyString {
    fn from(value: Cow<'a, str>) -> Self {
        Self(value.into())
    }
}

impl Borrow<str> for BunnyString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for BunnyString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl AsRef<[u8]> for BunnyString {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Deref for BunnyString {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl Display for BunnyString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.0.as_str(), f)
    }
}

impl fmt::Write for BunnyString {
    #[inline]
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.push_str(s);
        Ok(())
    }

    #[inline]
    fn write_char(&mut self, c: char) -> fmt::Result {
        self.push(c);
        Ok(())
    }
}

impl Serialize for BunnyString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for BunnyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer).map(From::from)
    }
}

impl IntoIterator for BunnyString {
    type Item = char;

    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromIterator<char> for BunnyString {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        iter.piped(String::from_iter).piped(Self::from)
    }
}

impl<'a> FromIterator<&'a char> for BunnyString {
    fn from_iter<T: IntoIterator<Item = &'a char>>(iter: T) -> Self {
        iter.piped(String::from_iter).piped(Self::from)
    }
}

impl TextBuffer for BunnyString {
    fn is_mutable(&self) -> bool {
        true
    }

    fn as_str(&self) -> &str {
        self.as_ref()
    }

    fn insert_text(&mut self, text: &str, char_index: usize) -> usize {
        let byte_idx = byte_index_from_char_index(self.as_str(), char_index);

        self.insert_str(byte_idx, text);

        text.chars().count()
    }

    fn delete_char_range(&mut self, char_range: std::ops::Range<usize>) {
        assert!(
            char_range.start <= char_range.end,
            "start must be <= end, but got {char_range:?}"
        );

        let byte_start = byte_index_from_char_index(self.as_str(), char_range.start);
        let byte_end = byte_index_from_char_index(self.as_str(), char_range.end);

        self.drain(byte_start..byte_end);
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn replace_with(&mut self, text: &str) {
        self.0 = RString::from(text)
    }

    fn take(&mut self) -> String {
        std::mem::take(&mut self.0).into_string()
    }

    fn type_id(&self) -> std::any::TypeId {
        std::any::TypeId::of::<Self>()
    }
}
