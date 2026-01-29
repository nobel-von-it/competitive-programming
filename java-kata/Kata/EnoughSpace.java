package Kata;

public class EnoughSpace {
    public static int enough(int cap, int on, int wait) {
        int n = cap - on - wait;
        return n > 0 ? 0 : Math.abs(n);
//        or best
//        return Math.max(0, wait + on - cap);
    }
}
