package Kata;

public class Logs {
    public static void main(String[] args) {
        System.out.println(logs(10, 2 ,3));
    }
    public static double logs(double x, double a, double b) {
        return (Math.log(a) + Math.log(b)) / Math.log(x);
    }
}
