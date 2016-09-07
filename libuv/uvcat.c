#include <stdio.h>
#include <assert.h>
#include <uv.h>

// the callbacks
void on_open(uv_fs_t* req);
void on_read(uv_fs_t* req);
void on_write(uv_fs_t* req);

// the requests
uv_fs_t open_req;
uv_fs_t read_req;
uv_fs_t write_req;

// used in on_open, read 1024 bits every time
static uv_buf_t iov;
static char buffer[1024];

int main(int argc, char** argv) {
    // http://docs.libuv.org/en/v1.x/fs.html#c.uv_fs_open
    // use uv_fs_open to get file discriptor, 
    // file discriptor is a small, nonnegative integer for use in subsequent system calls (read(2), write(2), lseek(2), fcntl(2), etc.).
    // TODO: why design like that? for context saving? Will other process's accessing to the file discriptor be prevented?
    //  see NOTES of open(2)
    // TODO: if err, errno will set. but why design errno is the last no?
    // uv_loop_t*
    // uv_fs_t*
    // const char* path
    // int flags:
    //  access mode: RD, WR, RDWR
    //  creation flags: CREAT stuff --- 
    //  status flags: ASYNC, APPEND stuff
    // int mode: follow CREAT, 777 thing
    // uv_fs_cb callback: been given the uv_fs_t*
    //  uv_fs_t has: http://docs.libuv.org/en/v1.x/fs.html#c.uv_fs_t.result
    uv_fs_open(uv_default_loop(), &open_req, argv[1], O_RDONLY, 0, on_open);


    uv_run(uv_default_loop(), UV_RUN_DEFAULT);

    uv_fs_req_cleanup(&open_req);
    uv_fs_req_cleanup(&read_req);
    uv_fs_req_cleanup(&write_req);
    return 0;
}

void on_open(uv_fs_t* req) {
    assert(req == &open_req);
    if (req->result < 0) {
        // http://docs.libuv.org/en/v1.x/fs.html#c.uv_fs_t.result
        fprintf(stderr, "When open, get %s: %s\n", uv_err_name((int)req->result), uv_strerror((int)req->result));   // TODO: how to print as err(make sh know that)?
    } else {
        iov = uv_buf_init(buffer, sizeof(buffer));
        // http://docs.libuv.org/en/v1.x/fs.html#c.uv_fs_read
        // http://man7.org/linux/man-pages/man2/readv.2.html --> the NAME section should be 'read from or write data into...'
        // preadv(2): read amount of data from file with offset and write it to buffer
        //  the amount defined by nbufs
        //  the file defined by file
        //  the offset defined by offset
        //  the buffer defined by uv_buf_t[]
        //  return the number of bytes read if success, or -1 if err
        // uv_loop_t*
        // uv_fs_t*
        // uv_file file
        // const uv_buf_t[]
        // unsigned int nbufs
        // int64_t offset
        // uv_fs_cb
        //  be give read_req*, whose result is the result of preadv
        uv_fs_read(uv_default_loop(), &read_req, req->result, &iov, 1, -1, on_read);
    }
}

void on_read(uv_fs_t* req) {
    // req is read_req, read_req->result indicates the amount of data read
    if (req->result < 0) {
        fprintf(stderr, "When read, get %s: %s\n", uv_err_name((int)req->result), uv_strerror((int)req->result));
    } else if (req->result == 0) {
        uv_fs_t close_req;
        uv_fs_close(uv_default_loop(), &close_req, open_req.result, NULL);
    } else {
        iov.len = req->result;
        //
        // uv_loop_t*
        // uv_fs_t*
        // uv_file
        //  https://en.wikipedia.org/wiki/File_descriptor
        //  1 is standard output
        uv_fs_write(uv_default_loop(), &write_req, 1, &iov, 1, -1, on_write);
    }
}

void on_write(uv_fs_t* req) {
    if (req->result < 0) {
        fprintf(stderr, "When write, get %s: %s\n", uv_err_name((int)req->result), uv_strerror((int)req->result));
    } else {
        uv_fs_read(uv_default_loop(), &read_req, open_req.result, &iov, 1, -1, on_read);
    }
}

