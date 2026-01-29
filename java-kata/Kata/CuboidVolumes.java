package Kata;

import java.util.Arrays;

public class CuboidVolumes {
    public static int findDifference(final int[] f, final int[] s) {
        return Math.abs(Arrays.stream(f).reduce((a, el) -> a * el).getAsInt() -
                Arrays.stream(s).reduce((a, el) -> a * el).getAsInt());
    }

    //or (best time 2x)
    public static int findDiff(final int[] f, final int[] s) {
        int fs = 1, ss = 1;
        for (int i = 0; i < 3; i++) {
            fs *= f[i];
            ss *= s[i];
        }
        return Math.abs(fs - ss);
    }
}
