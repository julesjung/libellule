//
//  Loadable.swift
//  Libellule
//
//  Created by Jules on 10/08/2026.
//

import Foundation

enum Loadable<T> {
    case idle
    case loading
    case loaded(T)
    case failed(Error)
}
