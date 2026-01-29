package Kata;

public class CountPosSumNeg {
    public static int[] countPositivesSumNegatives(int[] a) {
        if (a == null || a.length == 0) return new int[]{0,0};
        int sum = 0, count = 0;
        for (int i: a) {
            if (i > 0) count++;
            else sum += i;
        }
        return new int[]{count, sum};
    }
}
