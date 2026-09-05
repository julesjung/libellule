//
//  KeychainService.swift
//  Libellule
//
//  Created by Jules on 13/08/2026.
//

import Foundation
import Security

nonisolated final class KeychainService {
    static let shared = KeychainService()
    
    private init() {}
    
    func save(value: String, forKey: String) throws {
        try delete(forKey: forKey)
        
        let data = Data(value.utf8)
        
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: forKey,
            kSecValueData as String: data,
            kSecAttrAccessible as String: kSecAttrAccessibleWhenUnlockedThisDeviceOnly
        ]
        
        let status = SecItemAdd(query as CFDictionary, nil)
        
        guard status == errSecSuccess else {
            throw KeychainError.unableToSave(status)
        }
    }
    
    func load(forKey: String) throws -> String? {
            let query: [String: Any] = [
                kSecClass as String: kSecClassGenericPassword,
                kSecAttrAccount as String: forKey,
                kSecReturnData as String: true,
                kSecMatchLimit as String: kSecMatchLimitOne
            ]

            var result: AnyObject?
            let status = SecItemCopyMatching(
                query as CFDictionary,
                &result
            )

            if status == errSecItemNotFound {
                return nil
            }

            guard status == errSecSuccess,
                  let data = result as? Data,
                  let value = String(data: data, encoding: .utf8)
            else {
                throw KeychainError.unableToLoad(status)
            }

            return value
        }
    
    func delete(forKey: String) throws {
        let query: [String: Any] = [
            kSecClass as String: kSecClassGenericPassword,
            kSecAttrAccount as String: forKey
        ]

        let status = SecItemDelete(query as CFDictionary)

        guard status == errSecSuccess || status == errSecItemNotFound else {
            throw KeychainError.unableToDelete(status)
        }
    }
}

enum KeychainError: Error {
    case unableToSave(OSStatus)
    case unableToLoad(OSStatus)
    case unableToDelete(OSStatus)
}
