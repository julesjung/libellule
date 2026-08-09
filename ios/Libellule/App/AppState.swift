//
//  AppModel.swift
//  Libellule
//
//  Created by Jules on 09/08/2026.
//

import Foundation
import LibelluleKit

enum AppState {
    case loggedOut
    case connecting(url: String)
    case connected(instance: Instance)
    case authenticating(instance: Instance, username: String, password: String)
    case authenticated(client: Client)
}
