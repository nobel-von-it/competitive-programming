package Kata;

public class BlueRedMarbles {
    public static double guessBlue(int bs, int rs, int bp, int rp) {
        return (double) (bs - bp) / ((bs - bp) + (rs - rp));
    }
}
