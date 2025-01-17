// Part of the Concrete Compiler Project, under the BSD3 License with Zama
// Exceptions. See
// https://github.com/zama-ai/concrete-compiler-internal/blob/main/LICENSE.txt
// for license information.

#ifndef CONCRETELANG_C_DIALECT_FHE_H
#define CONCRETELANG_C_DIALECT_FHE_H

#include "mlir-c/IR.h"

#ifdef __cplusplus
extern "C" {
#endif

/// \brief structure to return an MlirType or report that there was an error
/// during type creation.
typedef struct {
  MlirType type;
  bool isError;
} MlirTypeOrError;

MLIR_DECLARE_CAPI_DIALECT_REGISTRATION(FHE, fhe);

/// Creates an encrypted integer type of `width` bits
MLIR_CAPI_EXPORTED MlirTypeOrError
fheEncryptedIntegerTypeGetChecked(MlirContext context, unsigned width);

/// If the type is an EncryptedInteger
MLIR_CAPI_EXPORTED bool fheTypeIsAnEncryptedIntegerType(MlirType);

/// Creates an encrypted signed integer type of `width` bits
MLIR_CAPI_EXPORTED MlirTypeOrError
fheEncryptedSignedIntegerTypeGetChecked(MlirContext context, unsigned width);

/// If the type is an EncryptedSignedInteger
MLIR_CAPI_EXPORTED bool fheTypeIsAnEncryptedSignedIntegerType(MlirType);

/// \brief Get bitwidth of the encrypted integer type.
///
/// \return bitwidth of the encrypted integer or 0 if it's not an encrypted
/// integer
MLIR_CAPI_EXPORTED unsigned fheTypeIntegerWidthGet(MlirType);

#ifdef __cplusplus
}
#endif

#endif // CONCRETELANG_C_DIALECT_FHE_H
