
#![allow(dead_code)]

use std::marker::PhantomData;
use tquery_derive::TQuery;
use tquery::Replace;
use typenum::*;

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

pub struct NotYetRead;
pub struct Read;
pub struct InvalidRow;
#[derive(TQuery)]
pub struct ValidRow<State>(PhantomData<State>);
#[derive(TQuery)]
pub struct CurrentRow<RowState>(PhantomData<RowState>);

pub struct Inserting;
pub struct Inserted;
#[derive(TQuery)]
pub struct InsertRow<InsState>(PhantomData<InsState>);

pub struct ReadOnly;
pub struct Updatable;
#[derive(TQuery)]
pub struct Concurrency<CcState>(PhantomData<CcState>);

pub struct Scrollable;
pub struct ForwardOnly;
#[derive(TQuery)]
pub struct Direction<DirState>(PhantomData<DirState>);
pub struct Closed;
#[derive(TQuery)]
pub struct Open<Position, Concurrency, Direction>(PhantomData<(Position, Concurrency, Direction)>);

#[derive(TQuery)]
struct ResultSet<SetState>(PhantomData<SetState>);

//type UpdateCurrentRow2<RS, T> = Replace<RS, SetState.Position.RowState, T>;
type UpdateCurrentRow<RS, T> = Replace<RS, (TSetState, (TPosition, (TRowState, ()))), T>;
type T1 = UpdateCurrentRow<
    ResultSet<Open<CurrentRow<ValidRow<NotYetRead>>, ReadOnly, ForwardOnly>>, 
    InvalidRow>;
type T2 = ResultSet<Open<CurrentRow<InvalidRow>, ReadOnly, ForwardOnly>>;

fn test_ty_eq() {
    assert_type_eq!(T1, T2);
}

impl<S, C, D> ResultSet<Open<CurrentRow<S>, C, D>> {
    pub fn next(
        mut self,
    ) -> Either<
        UpdateCurrentRow<Self, ValidRow<NotYetRead>>,
        UpdateCurrentRow<Self, InvalidRow>
    > {
        panic!()
    }
}