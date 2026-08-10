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
    
    var state: State = .loggedOut
    
    func connect(url: String) async {
        state = .connecting
        
        do {
            let instance = try await Instance(url: url)
            UserDefaults.standard.set(url, forKey: "url")
            state = .connected(instance)
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
        }
    }
}

enum SessionError: Error {
    case connection(Error)
    case authentication(Error)
}
