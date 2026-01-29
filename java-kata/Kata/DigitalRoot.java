package Kata;

import java.util.Arrays;

public class DigitalRoot {
    public static int digital_root(int n) {
        while (n >= 10) {
            String[] s = String.valueOf(n).split("");
            int[] is = new int[s.length];
            for (int i = 0; i < s.length; i++) {
                is[i] = Integer.parseInt(s[i]);
            }
            n = Arrays.stream(is).sum();
        }
        return n;
    }
    // or math
    public static int diRoot(int n) {
        return (n != 0 && n % 9 == 0) ? 9 : n % 9;
    }

}
