package se.hof.adventofcode.eighteen;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

public class DayEightTest {

    @Test
    public void parseChildlessNodeTest() {
        String input = "0 1 99";
        DayEight.Node n = DayEight.Node.parse(input);

        int expectedSum = 99;
        assertEquals(expectedSum, n.sum());
    }

    @Test
    public void parseChildlessNodeTest2() {
        String s = "0 3 10 11 12";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(33, n.sum());
    }


    @Test
    public void parseNodeWithChild() {
        String s = "1 1 0 1 99 2";
        DayEight.Node n = DayEight.Node.parse(s);

        assertEquals(101, n.sum());
    }

    @Test
    public void parseNodeWithNChildren() {
        String s = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        verifyNode(s, 2, 138);
    }

    @Test
    public void parseNodeWithNoChildrenAndNoMetadata() {
        String s = "0 0";
        verifyNode(s, 0, 0);
    }

    @Test
    public void parseNodeWith3Children() {
        String s = "3 3 0 3 10 11 12 0 1 2 0 1 99 1 1 2";
        verifyNode(s, 3, 138);
    }

    public static void verifyNode(String s, int expectedNrOfChildren, int expectedSum) {
        DayEight.Node n = DayEight.Node.parse(s);
        assertEquals(expectedNrOfChildren, n.children.size());
        assertEquals(expectedSum, n.sum());
    }

    @Test
    public void valueNoChildrenTest() {
        String input = "0 1 99";
        DayEight.Node n = DayEight.Node.parse(input);

        int expectedSum = 99;
        assertEquals(expectedSum, n.value());
    }

    @Test
    public void valueMetadatNoRefChildTest() {
        String input = "1 1 0 1 99 2";
        DayEight.Node n = DayEight.Node.parse(input);

        int expectedSum = 0;
        assertEquals(expectedSum, n.value());
    }

    @Test
    public void valueMetadatRefChildTest() {
        String input = "1 1 0 1 99 1";
        DayEight.Node n = DayEight.Node.parse(input);

        int expectedSum = 99;
        assertEquals(expectedSum, n.value());
    }

    @Test
    public void valueTest() {
        String input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
        DayEight.Node n = DayEight.Node.parse(input);

        int expectedSum = 66;
        assertEquals(expectedSum, n.value());
    }
}
