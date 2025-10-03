pub trait Pager {
    type Order;

    fn max_results(self, items: usize) -> Self;
    fn order(self, order: Self::Order) -> Self;
}
