package tasks

import contributors.GitHubService
import contributors.RequestData
import contributors.User
import contributors.log
import contributors.logRepos
import contributors.logUsers
import java.util.concurrent.atomic.AtomicInteger
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response

fun loadContributorsCallbacks(service: GitHubService, req: RequestData, updateResults: (List<User>) -> Unit) {
    service.getOrgReposCall(req.org).onResponse { responseRepos ->
        logRepos(req, responseRepos)
        val repos = responseRepos.bodyList()
        val allUsers = mutableListOf<User>()
        var count = AtomicInteger()
        for (repo in repos) {
            service.getRepoContributorsCall(req.org, repo.name).onResponse { responseUsers ->
                logUsers(repo, responseUsers)
                val users = responseUsers.bodyList()
                allUsers += users
                if (count.incrementAndGet() == repos.size) {
                    updateResults(allUsers.aggregate())
                }
            }
        }
    }
}

inline fun <T> Call<T>.onResponse(crossinline callback: (Response<T>) -> Unit) {
    enqueue(object : Callback<T> {
        override fun onResponse(call: Call<T>, response: Response<T>) {
            callback(response)
        }

        override fun onFailure(call: Call<T>, t: Throwable) {
            log.error("Call failed", t)
        }
    })
}
