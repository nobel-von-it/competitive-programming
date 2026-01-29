package Kata;

public class Quadro {
    public static void main(String[] args) {
        System.out.println(quadrant(1,2));
    }
    public static int quadrant(int x, int y) {
        return x > 0 ? y > 0 ? 1 : 4 : y > 0 ? 2 : 3;
    }
}
