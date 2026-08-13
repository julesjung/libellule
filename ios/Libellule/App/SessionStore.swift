//
//  SessionStore.swift
//  Libellule
//
//  Created by Jules on 10/08/2026.
//

import Foundation
import LibelluleKit

@Observable
final class SessionStore {
    enum State {
        case loggedOut
        case connecting
        case connected(Instance)
        case authenticating
        case authenticated(Client)
        case failed(SessionError)
    }
    
    var state: State = .connecting
    
    func connect(url: String) async {
        state = .connecting
        
        do {
            let instance = try await Instance(url: url)
            UserDefaults.standard.set(url, forKey: "url")
            state = .authenticating
            
            guard let username = try? KeychainService.shared.load(forKey: "username"),
                  let password = try? KeychainService.shared.load(forKey: "password")
            else {
                state = .connected(instance)
                return
            }
            
            await login(instance: instance, username: username, password: password)
        } catch {
            state = .failed(.connection(error))
        }
    }
    
    func login(instance: Instance, username: String, password: String) async {
        state = .authenticating
        
        do {
            let client = try await Client(instance: instance, username: username, password: password)
            state = .authenticated(client)
        } catch {
            state = .failed(.authentication(error))
            return
        }
        
        try? KeychainService.shared.save(value: username, forKey: "username")
        try? KeychainService.shared.save(value: password, forKey: "password")
    }
}

enum SessionError: Error {
    case connection(Error)
    case authentication(Error)
}
