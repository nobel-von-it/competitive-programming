package Kata;

public class FibLastNum {
    public static int getFibNumb(int n) {
        int l = 0, m = 1;
        for (int i = 2; i < n; i++) {
            int t = (l + m) % 10;
            l = m;
            m = t;
        }
        return m;
    }
}
