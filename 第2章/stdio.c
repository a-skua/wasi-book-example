#include <sys/uio.h>

int main() {
    char name[100];
    struct iovec iov[4];
    iov[0].iov_base = "Your name: ";
    iov[0].iov_len = 11;
    iov[1].iov_base = "Hello, ";
    iov[1].iov_len = 7;
    iov[2].iov_base = name;
    iov[2].iov_len = 100;
    iov[3].iov_base = "!\n";
    iov[3].iov_len = 2;

    writev(1, iov, 1);

    // 標準入力
    int len = readv(0, iov + 2, 1);
    iov[2].iov_len = len - 1; // 改行文字を含めない

    // 標準出力
    writev(1, iov + 1, 3);
    return 0;
}
