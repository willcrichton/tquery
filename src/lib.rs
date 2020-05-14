pub trait ComputeGetType<Idx> { type Output; }
pub type GetType<T, Idx> = <T as ComputeGetType<Idx>>::Output;

pub trait ComputeSetType<Idx, NewT> { type Output; }
pub type SetType<T, Idx, NewT> = <T as ComputeSetType<Idx, NewT>>::Output;

pub trait ComputeReplace<Selector, Replacement> { type Output; }
pub type Replace<T, Selector, Replacement> =
  <T as ComputeReplace<Selector, Replacement>>::Output;

impl<Replacement, T> ComputeReplace<(), Replacement> for T {
  type Output = Replacement;
}

impl<Sel, SelList, Replacement, T> ComputeReplace<(Sel, SelList), Replacement> for T
where
  T: ComputeGetType<Sel>,
  GetType<T, Sel>: ComputeReplace<SelList, Replacement>,
  T: ComputeSetType<Sel, Replace<GetType<T, Sel>, SelList, Replacement>>
{
  type Output = SetType<T, Sel, Replace<GetType<T, Sel>, SelList, Replacement>>;
}