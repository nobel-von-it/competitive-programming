package Kata;

public class Theater {
    public static int seatsInTheater(int nCols, int nRows, int col, int row) {
        return (nRows - row) * (nCols + 1 - col);
    }
}
