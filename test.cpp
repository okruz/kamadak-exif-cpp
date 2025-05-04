/*
 * ########################################################################
 * The contents of this file is free and unencumbered software released into the
 * public domain. For more information, please refer to <http://unlicense.org/>
 * ########################################################################
 */

#include <fstream>
#include <iostream>
#include <vector>

#include "kamadak_exif_cpp.h"

static std::ostream& operator<<(std::ostream& o, EXIF_ErrorCodes e) {
  switch (e) {
    case EXIF_ErrorCodes::Ok:
      o << "[Ok]";
      break;
    case EXIF_ErrorCodes::Nullptr:
      o << "[Nullptr]";
      break;
    case EXIF_ErrorCodes::ParseError:
      o << "[ParseError]";
      break;
    default:
      o << "[Unknown]";
      break;
  }
  return o;
}

static std::vector<char> readFile() {
  std::ifstream file("images/Peak-in-kuh-e-genu-mountain-range-iran.jpg");

  return std::vector<char>{std::istreambuf_iterator<char>(file),
                           std::istreambuf_iterator<char>()};
}

int main() {
  const auto data = readFile();
  auto parseResult =
      EXIF_load(reinterpret_cast<const uint8_t*>(data.data()), data.size());
  std::cout << "reinterpret_cast<const uint8_t*>(data.data()), data.size(): "
            << parseResult.error_code << std::endl;
  bool littleEndian = false;

  std::cout << "is_little_endian: "
            << EXIF_is_little_endian(parseResult.data, &littleEndian) << " "
            << std::boolalpha << littleEndian << std::endl;

  const EXIF_KeyValuePair* entries;
  size_t num;
  EXIF_load_entries(parseResult.data, &entries, &num);
  std::cout << "Found " << num << " entries:\n";
  for (size_t i = 0; i < num; i++) {
    std::cout << entries[i].key << ": " << entries[i].value << "\n";
  }
  std::cout << std::endl;

  std::cout << "free_exif(parseResult.data): " << EXIF_free(parseResult.data)
            << std::endl;

  return 0;
}
