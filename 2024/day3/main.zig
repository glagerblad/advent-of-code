const std = @import("std");

const printf = std.debug.print;

const Result = struct {
    match: bool,
    x: i32,
    y: i32,
};
const DoDont = struct {
    match: bool,
    value: bool,
};

const PREFIX = "mul(";
const POSTFIX = ")";
const DELIMITER = ",";
const MIN_SIZE = PREFIX.len + 1 + DELIMITER.len + 1 + POSTFIX.len;
const MAX_SIZE = PREFIX.len + 3 + DELIMITER.len + 3 + POSTFIX.len;

const FILE_PATH = "input.txt";

const DO = "do()";
const DONT = "don't()";

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    var file = try std.fs.cwd().openFile(FILE_PATH, .{});
    defer file.close();

    var buffer = std.io.bufferedReader(file.reader());
    var reader = buffer.reader();

    var arr = std.ArrayList(u8).init(allocator);
    defer arr.deinit();

    var acc: i32 = 0;
    var enabled = true;

    while (true) {
        reader.streamUntilDelimiter(arr.writer(), '\n', null) catch |err| switch (err) {
            error.EndOfStream => break,
            else => return err,
        };
        const line = arr.items;
        var i: usize = 0;
        while (i <= line.len) {
            const chunk = if (line.len > MAX_SIZE + i) line[i .. i + MAX_SIZE] else line[i..];
            const result_mul = try extract_values(chunk);
            const result_do_dont = try extract_do_dont(chunk);
            if (enabled and result_mul.match) acc += result_mul.x * result_mul.y;
            if (result_do_dont.match) enabled = result_do_dont.value;
            i += 1;
        }

        arr.clearRetainingCapacity();
    }
    printf("Result: {d}\n", .{acc});
}

fn extract_do_dont(chunk: []const u8) !DoDont {
    if (chunk.len > DO.len and std.mem.eql(u8, chunk[0..DO.len], DO)) return DoDont{ .match = true, .value = true };
    if (chunk.len > DONT.len and std.mem.eql(u8, chunk[0..DONT.len], DONT)) return DoDont{ .match = true, .value = false };
    return DoDont{ .match = false, .value = false };
}

fn extract_values(chunk: []const u8) !Result {
    if (chunk.len <= MIN_SIZE or !std.mem.eql(u8, chunk[0..PREFIX.len], PREFIX)) {
        return Result{ .match = false, .x = 0, .y = 0 };
    }
    const delimiter_pos: usize = std.mem.indexOf(u8, chunk, DELIMITER) orelse 0;
    const postfix_pos: usize = std.mem.indexOf(u8, chunk, POSTFIX) orelse 0;
    if (delimiter_pos == 0 or postfix_pos == 0) return Result{ .match = false, .x = 0, .y = 0 };
    const x_str = chunk[PREFIX.len..delimiter_pos];
    const y_str = chunk[delimiter_pos + 1 .. postfix_pos];
    const x = try std.fmt.parseInt(i32, x_str, 10);
    const y = try std.fmt.parseInt(i32, y_str, 10);
    return Result{ .match = true, .x = x, .y = y };
}
