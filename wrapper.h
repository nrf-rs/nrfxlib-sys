/**
 * These are the header files exported by this crate.
 */

/*
 * libbsd headers
 */
#include "nrf_modem/include/nrf_modem.h"
#include "nrf_modem/include/nrf_modem_limits.h"
#include "nrf_modem/include/nrf_modem_full_dfu.h"
#include "nrf_modem/include/nrf_modem_os.h"
#include "nrf_modem/include/nrf_modem_platform.h"
#include "nrf_modem/include/nrf_errno.h"
#include "nrf_modem/include/nrf_socket.h"

/*
 * Crypto Cell 310 (CC310) platform headers
 */

#include "crypto/nrf_cc310_platform/include/nrf_cc3xx_platform.h"
#include "crypto/nrf_cc310_platform/include/nrf_cc3xx_platform_abort.h"
#include "crypto/nrf_cc310_platform/include/nrf_cc3xx_platform_defines.h"
#include "crypto/nrf_cc310_platform/include/nrf_cc3xx_platform_entropy.h"
#include "crypto/nrf_cc310_platform/include/nrf_cc3xx_platform_mutex.h"

/*
 * Crypto Cell 310 (CC310) mbedTLS integration headers
 *
 * Note: We only include the sub-set of header files that don't require
 * access to the full mbedTLS headers (which aren't bundled in nrfxlib).
 */

#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/aes_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/chacha20_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/chachapoly_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/cmac_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/dhm_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/ecp_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/platform_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/poly1305_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/rsa_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/sha1_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/sha256_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls/threading_alt.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_aes_defs.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_aes_defs_proj.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_bitops.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_error.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_hash_defs.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_hash_defs_proj.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_kdf.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_pal_compiler.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_pal_types.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_pal_types_plat.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_rnd_common.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/cc_rnd_error.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/mbedtls_cc_aes_key_wrap.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/mbedtls_cc_aes_key_wrap_error.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/mbedtls_cc_ec_mont_edw_error.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/mbedtls_cc_hkdf.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/mbedtls_cc_hkdf_error.h"
#include "crypto/nrf_cc310_mbedcrypto/include/mbedtls_extra/mbedtls_cc_srp_error.h"

/*
 * In addition to CC310 acceleration for mbedTLS, this repository also appears
 * to contain pre-compiled crypto code from https://oberonhap.com. These
 * are the liboberon headers.
 */

#include "crypto/nrf_oberon/include/ocrypto_aes_ctr.h"
#include "crypto/nrf_oberon/include/ocrypto_aes_eax.h"
#include "crypto/nrf_oberon/include/ocrypto_aes_gcm.h"
#include "crypto/nrf_oberon/include/ocrypto_aes_key.h"
#include "crypto/nrf_oberon/include/ocrypto_chacha20.h"
#include "crypto/nrf_oberon/include/ocrypto_chacha20_poly1305.h"
#include "crypto/nrf_oberon/include/ocrypto_chacha20_poly1305_inc.h"
#include "crypto/nrf_oberon/include/ocrypto_constant_time.h"
#include "crypto/nrf_oberon/include/ocrypto_curve25519.h"
// #include "crypto/nrf_oberon/include/ocrypto_curve_p256.h"
#include "crypto/nrf_oberon/include/ocrypto_ecdh_p256.h"
#include "crypto/nrf_oberon/include/ocrypto_ecdsa_p256.h"
#include "crypto/nrf_oberon/include/ocrypto_ed25519.h"
#include "crypto/nrf_oberon/include/ocrypto_hkdf_sha256.h"
#include "crypto/nrf_oberon/include/ocrypto_hkdf_sha512.h"
#include "crypto/nrf_oberon/include/ocrypto_hmac_sha256.h"
#include "crypto/nrf_oberon/include/ocrypto_hmac_sha512.h"
#include "crypto/nrf_oberon/include/ocrypto_rsa.h"
#include "crypto/nrf_oberon/include/ocrypto_rsa_key.h"
#include "crypto/nrf_oberon/include/ocrypto_sha1.h"
#include "crypto/nrf_oberon/include/ocrypto_sha256.h"
#include "crypto/nrf_oberon/include/ocrypto_sha512.h"
#include "crypto/nrf_oberon/include/ocrypto_srp.h"
#include "crypto/nrf_oberon/include/ocrypto_srtp.h"

// End of File
