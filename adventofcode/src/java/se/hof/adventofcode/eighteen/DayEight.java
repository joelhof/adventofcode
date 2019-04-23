package se.hof.adventofcode.eighteen;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;

/**
 * Try to solve Day 8 with java, for comparision...
 */
public class DayEight {

    public static void main(String[] args) {

        try {
            String input = Files.readString(Paths.get("resources", "eighteen", "dayEight.txt"));
            System.out.println("Input length: " + input.replace(" ", "").length());
            Node root = Node.parse(input);
            System.out.println("The total sum is: " + root.sum());
            int complexSum = root.value();
            System.out.println("The root node value is: " + complexSum);

        } catch (IOException e) {
            System.out.println("Failed to read input data");
            e.printStackTrace();
        }

    }

    public static class Node {
        public static final int NR_OF_HEADERS = 2;
        int[] headers;
        List<Node> children;
        int[] metadata;

        private Node (final int[] input) {
            System.out.println("Creating Node from " + Arrays.toString(input));
            this.headers = Arrays.copyOfRange(input, 0, NR_OF_HEADERS);
            this.children = new ArrayList<>();
            this.metadata = new int[headers[1]];
        }

        public int sum() {
            return sum(this);
        }

        public int value() {
            return children.size() > 0
                    ? Arrays.stream(metadata)
                    .map(i -> --i)
                    .filter(i -> i < children.size())
                    .mapToObj(i -> children.get(i))
                    .mapToInt(n -> n.value())
                    .sum()
                    : Arrays.stream(metadata).sum();
        }

        @Override
        public String toString() {
            return "Node{" +
                    "headers=" + Arrays.toString(headers) +
                    ", children=" + children +
                    ", metadata=" + Arrays.toString(metadata) +
                    '}';
        }

        private static int sum(Node node) {
            return Arrays.stream(node.metadata)
                    .sum()
                    + node.children.stream()
                    .mapToInt(c -> sum(c)).sum();
        }

        public static Node parse(String s) {
            String[] input = s.trim().split(" ");
            int[] arg = Arrays.stream(input)
                    .mapToInt(Integer::valueOf)
                    .toArray();
            Parser p = new Parser();
            return p.parse(arg);
        }
    }

    public static class Parser {

        int p = 0;

        Node parse(int[] input) {
            Node parent = new Node(Arrays.copyOfRange(input, p, input.length));
            p += Node.NR_OF_HEADERS;
            if (parent.headers[0] > 0) {
                for (int i = 0; i < parent.headers[0]; i++) {
                    Node child = parse(input);
                    parent.children.add(child);
                }
//                System.out.println("p after children " + p + " metdadata " + Arrays.toString(
//                        Arrays.copyOfRange(input, p, p + parent.headers[1])));
            }
            parent.metadata = Arrays.copyOfRange(input, p, p + parent.headers[1]);
            p += parent.headers[1];
//            System.out.println(parent.toString());
            return parent;
        }
    }
}
