import SwiftUI

struct ContentView: View {
    var lib = InteropLib()
    
    var body: some View {
        VStack
        {
            Text("Hello, world!")
                .padding()
            
            Button(action: {
                lib.call()
            }) {
                Text("CRASH!")
            }
            
            Spacer()
        }
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
