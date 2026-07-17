//
//  ContentView.swift
//  Pronote
//
//  Created by Jules on 17/07/2026.
//

import SwiftUI
import PronoteKit

struct ContentView: View {
    @State private var client: Client?
    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text("Hello, world!")
        }
        .padding()
        .task {
            client = try! await Client(instanceUrl: "https://demo.index-education.net/pronote/")
            
            try! await client?.connect()
        }
    }
}

#Preview {
    ContentView()
}
