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

    public boolean equals(Object o){
        if (o.getClass() != this.getClass()){
            return false;
        }

        Tuple<?, ?> p = (Tuple<?,?>) o;
        return (this.left == p.get_left() && this.right == p.get_right());
    }

//    public int hashCode() {
//        if (this.left == null || this.right == null){
//            return 0;
//        }
//
//        int ret = 991;
//        ret = 433 * ret + left.hashCode();
//        ret = 433 * ret + right.hashCode();
//        return ret;
//    }

    public String toString(){
        return ("{" + this.left + ", " + this.right + "}");
    }
}

