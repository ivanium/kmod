#include <linux/module.h>
#include <linux/slab.h>

MODULE_AUTHOR("Sascha Grunert <mail@saschagrunert.de> ivanium modified");
MODULE_DESCRIPTION("A simple kernel module");
MODULE_LICENSE("MIT");
MODULE_VERSION("0.1.1");

// The entry and exit function
extern int init_module(void);
extern void cleanup_module(void);

extern void add(int a, int b, int *c);
extern void mul(int a, int b, int *c);

// void add(int a, int b, int *c) {
//     *c = a + b;
// }
// void mul(int a, int b, int *c) {
//     *c = a * b;
// }