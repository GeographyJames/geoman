import { Users, AlertCircle } from "lucide-react";
import { useUsers } from "@/hooks/api/useUsers";
import { useCurrentUser } from "@/hooks/api/useCurrentUser";

export default function TeamsSection() {
  const { data: currentUser } = useCurrentUser();
  const { data: users = [] } = useUsers();
  if (!currentUser) return <></>;

  const teamMembers = users.filter(
    (user) => user?.teamId === currentUser.teamId,
  );

  return (
    <>
      <div className="mb-6">
        <h1 className="text-2xl font-semibold mb-1">Teams</h1>
        <p className="text-base-content/70">
          Manage your team and team members
        </p>
      </div>

      <div className="card bg-base-100 border border-base-300">
        <div className="card-body">
          <h2 className="card-title text-lg flex items-center gap-2">
            <Users size={20} />
            Team Information
          </h2>
          {currentUser.team ? (
            <div className="flex flex-col gap-4">
              <div className="flex items-center gap-2">
                <span className="text-base-content/70">Team:</span>
                <span className="font-semibold">{currentUser.team.name}</span>
              </div>

              {teamMembers.length > 0 && (
                <div>
                  <h3 className="text-sm font-semibold text-base-content/70 mb-2">
                    Team Members ({teamMembers.length})
                  </h3>
                  <div className="space-y-2">
                    {teamMembers.map((member) => (
                      <div
                        key={member.id}
                        className={`flex items-center gap-2 p-2 rounded ${
                          member.id === currentUser.id
                            ? "bg-primary/10 border border-primary/20"
                            : "bg-base-200"
                        }`}
                      >
                        <div className="avatar placeholder">
                          <div className="bg-neutral text-neutral-content rounded-full w-8">
                            <span className="text-xs">
                              {member.firstName[0]}
                              {member.lastName[0]}
                            </span>
                          </div>
                        </div>
                        <div className="flex-1">
                          <div className="font-medium">
                            {member.firstName} {member.lastName}
                            {member.id === currentUser.id && (
                              <span className="ml-2 text-xs text-primary">
                                (You)
                              </span>
                            )}
                          </div>
                        </div>
                      </div>
                    ))}
                  </div>
                </div>
              )}
            </div>
          ) : (
            <div className="alert">
              <AlertCircle size={20} />
              <div>
                <h3 className="font-semibold">No team assigned</h3>
                <div className="text-sm">
                  You are not currently assigned to a team. Contact your
                  administrator to join a team.
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </>
  );
}
