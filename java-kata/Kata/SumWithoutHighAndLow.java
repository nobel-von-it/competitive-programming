package Kata;

public class SumWithoutHighAndLow {
    public static int sum(int[] a) {
        if (a == null || a.length <= 1) return 0;
        int max = -1, min = 100000, sum = 0;
        for (int i: a) {
            if (max < i) max = i;
            if (min > i) min = i;
            sum += i;
        }
        return sum - max - min;
    }
}
