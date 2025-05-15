use std::{ffi::c_void, fmt, marker::PhantomData};

use crate::{Rect, ffi::*};

/// Dirty rectangles for CEF.
pub struct DirtyRects<'a> {
    ptr: *const c_void,
    _mark: PhantomData<&'a ()>,
}

impl fmt::Debug for DirtyRects<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ls = f.debug_list();
        for rect in self.iter() {
            ls.entry(&rect);
        }
        ls.finish()
    }
}

impl<'a> DirtyRects<'a> {
    #[inline]
    pub(crate) fn new(dirty_rects: *const c_void) -> Self {
        DirtyRects {
            ptr: dirty_rects,
            _mark: PhantomData,
        }
    }

    /// Returns the number of dirty rectangles.
    #[inline]
    pub fn len(&self) -> usize {
        unsafe { wef_dirty_rects_len(self.ptr) as usize }
    }

    /// Returns `true` if there are no dirty rectangles.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the dirty rectangle at the specified index.
    #[inline]
    pub fn get(&self, i: usize) -> Rect {
        unsafe {
            debug_assert!(i < self.len(), "index out of bounds");
            let mut rect = Rect::default();
            wef_dirty_rects_get(self.ptr, i as i32, &mut rect);
            rect
        }
    }

    /// Returns an iterator over the dirty rectangles.
    #[inline]
    pub fn iter(&self) -> DirtyRectsIter<'a> {
        DirtyRectsIter {
            ptr: self.ptr,
            index: 0,
            _mark: PhantomData,
        }
    }
}

impl<'a> IntoIterator for &'a DirtyRects<'a> {
    type Item = Rect;
    type IntoIter = DirtyRectsIter<'a>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// An iterator over the dirty rectangles.
pub struct DirtyRectsIter<'a> {
    ptr: *const c_void,
    index: usize,
    _mark: PhantomData<&'a ()>,
}

impl Iterator for DirtyRectsIter<'_> {
    type Item = Rect;

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if self.index < wef_dirty_rects_len(self.ptr) as usize {
                let mut rect = Rect::default();
                wef_dirty_rects_get(self.ptr, self.index as i32, &mut rect);
                self.index += 1;
                Some(rect)
            } else {
                None
            }
        }
    }
}
