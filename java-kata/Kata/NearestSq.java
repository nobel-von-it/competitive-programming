package Kata;

public class NearestSq {
    public static void main(String[] args) {
        System.out.println(nearestSq(10));
    }
    public static int nearestSq(final int n) {
        return (int) Math.pow(Math.round(Math.sqrt(n)), 2);
    }
}
