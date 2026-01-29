package Kata;

public class FuelCalc {
    public static double fuelPrice(int l, double p) {
        System.out.println((int)(p * 100) / 100.0);
        return l > 10 ? (p * 100 - 25) * l / 100.0 : (p * 100 - l / 2 * 5) * l / 100.0;
    }
}
