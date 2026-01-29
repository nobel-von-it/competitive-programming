package Kata;

public class DutyFree {
    public static int dutyFree(int n, int d, int h) {
        return (int) Math.floor(h / (n * (d / 100.0)));
    }
}
