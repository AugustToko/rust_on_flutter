/**
 *
 * network module for `lgj.wlwsx.client.flutter`
 *
 */
char *rust_greeting(const char *to);

void rust_cstr_free(char *s);

char *net_test(void);

void modbus_write(CString ip, const uint8_t (*data)[8]);