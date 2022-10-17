public class Tuple<L, R> {
    private L left;
    private R right;

    public Tuple(L left, R right) {
        this.left = left;
        this.right = right;
    }

    public L get_left(){
        return left;
    }

    public R get_right(){
        return right;
    }
}
