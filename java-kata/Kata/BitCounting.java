package Kata;

import java.util.Arrays;

public class BitCounting {
    public static int countBits(int n) {
        if (n <= 0)
            return 0;
        String bin = "";
        while (n > 0) {
            bin += (n % 2);
            n /= 2;
        }
        return Arrays.stream(bin.split("")).mapToInt(Integer::parseInt).sum();
    }
    // Integer have this function yet ((
    public static int countBit(int n) {
        return Integer.bitCount(n);
    }
    // or just simply
    public static int countBitt(int n) {
        int res = n % 2;
        while ((n /= 2) > 0) res += n % 2;
        return res;
    }
}
